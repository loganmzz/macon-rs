//! Release helper:
//! * Check working directory is clean
//! * Update links to point release tag
//! * Update Cargo version
//! * Update changelog
//! * Git stage updated files
//! * Restore links to point to master
//! * Prepare next Cargo version
//! * Git commit released files
//! * Git tag release
//! * Git stage post-release files
//! * Git commit post-release files
//!

use std::{
    collections::HashMap,
    convert,
    ffi::OsStr,
    fmt,
    fs,
    process,
    str,
};
use anyhow::{
    bail,
    Context,
    Result,
};

struct Git;

impl Git {
    fn new() -> Self {
        Self
    }

    fn is_clean(&self) -> Result<bool> {
        let status = process::Command::new("git")
            .arg("diff")
            .arg("--exit-code")
            .output()
            .context("Unable to check Git working directory is clean")?
            .status;
        if ! status.success() {
            return Ok(false);
        }
        let status = process::Command::new("git")
            .arg("diff")
            .arg("--cached")
            .arg("--exit-code")
            .output()
            .context("Unable to check Git staging is clean")?
            .status;
        Ok(status.success())
    }

    fn add(&self, filenames: &[String]) -> Result<()> {
        let mut command = process::Command::new("git");
        command.arg("add");

        for filename in filenames {
            command.arg(filename);
        }

        let output = command
            .output()
            .context("Unable to add files to Git index")?;
        if ! output.status.success() {
            let stderr = ::std::str::from_utf8(&output.stderr)
                .context("Unable to parse UTF-8 for stderr")?;
            bail!("Unable to add files to Git index: {}", stderr);
        }
        Ok(())
    }

    fn commit<S: AsRef<OsStr>>(&self, message: S) -> Result<()> {
        let output = process::Command::new("git")
            .arg("commit")
            .arg("--message")
            .arg(message)
            .output()
            .context("Unable to Git commit")?;
        if ! output.status.success() {
            let stderr = ::std::str::from_utf8(&output.stderr)
                .context("Unable to parse UTF-8 for stderr")?;
            bail!("Unable to Git commit: {}", stderr);
        }
        Ok(())
    }

    fn tag<S: AsRef<OsStr>>(&self, tag: S) -> Result<()> {
        let output = process::Command::new("git")
            .arg("tag")
            .arg(tag)
            .output()
            .context("Unable to Git tag")?;
        if ! output.status.success() {
            let stderr = ::std::str::from_utf8(&output.stderr)
                .context("Unable to parse UTF-8 for stderr")?;
            bail!("Unable to Git tag: {}", stderr);
        }
        Ok(())
    }
}

#[derive(Clone,Debug,Default,PartialEq,)]
struct Version(String);

impl Version {
    fn new<T: convert::Into<String>>(version: T) -> Self {
        Self(version.into())
    }

    fn value(&self) -> &String {
        &self.0
    }
}

impl convert::From<semver::Version> for Version {
    fn from(value: semver::Version) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone,Copy,PartialEq,)]
enum Matching {
    Line,
    Exact,
}

struct Template {
    matching: Matching,
    source: String,
    target: String,
}

impl Template {
    fn from_single<T: convert::Into<String>>(template: T) -> Self {
        let source = template.into();
        let target = source.clone();
        Self::from_pair(source, target)
    }
    fn from_pair<S: convert::Into<String>, T: convert::Into<String>>(source: S, target: T) -> Self {
        Self {
            matching: Matching::Exact,
            source: source.into(),
            target: target.into(),
        }
    }
    fn set_matching(&mut self, matching: Matching) -> &mut Self {
        self.matching = matching;
        self
    }

    fn compile(&self, source_version: &Version, target_version: &Version) -> CompiledTemplate {
        let source = self.source.replace("{version}", source_version.value());
        let target = self.target.replace("{version}", target_version.value());
        CompiledTemplate {
            matching: self.matching,
            source,
            target,
        }
    }

    fn reverse(&self) -> Self {
        let mut reversed = Self::from_pair(&self.target, &self.source);
        reversed.set_matching(self.matching);
        reversed
    }
}

impl<S: convert::Into<String>, T: convert::Into<String>> convert::From<(S,T)> for Template {
    fn from(value: (S,T)) -> Self {
        Self::from_pair(value.0, value.1)
    }
}

impl convert::From<&str> for Template {
    fn from(value: &str) -> Self {
        Self::from_single(value)
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.source, self.target)
    }
}

struct CompiledTemplate {
    matching: Matching,
    source: String,
    target: String,
}

impl CompiledTemplate {
    fn replace(&self, text: &mut String) {
        *text =  if self.matching == Matching::Line {
            text.replace(&format!("\n{}\n", self.source), &format!("\n{}\n", self.target))
        } else {
            text.replace(&self.source, &self.target)
        };
    }
}

impl fmt::Display for CompiledTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.source, self.target)
    }
}

#[derive(Default,)]
struct TemplateSet {
    templates: Vec<(String, Template)>,
}

impl TemplateSet {
    fn add<N: convert::Into<String>, T: convert::Into<Template>>(&mut self, name: N, template: T, matching: Matching) -> &mut Self {
        let name = name.into();
        self.templates.retain(|(existing,_)| existing != &name);
        let mut template = template.into();
        template.set_matching(matching);
        self.templates.push((name, template));
        self
    }

    fn register<N: convert::Into<String>, T: convert::Into<Template>>(mut self, name: N, template: T, matching: Matching) -> Self {
        self.add(name, template, matching);
        self
    }

    fn compile(&self, source_version: &Version, target_version: &Version) -> CompiledTemplateSet {
        CompiledTemplateSet {
            templates: self.templates
                .iter()
                .map(|(name, template)| (
                    name.clone(),
                    template.compile(source_version, target_version),
                ))
                .collect(),
        }
    }

    fn reverse(&self) -> TemplateSet {
        TemplateSet {
            templates: self.templates
                .iter()
                .map(|(name, template)| (
                    name.clone(),
                    template.reverse(),
                ))
                .collect(),
        }
    }
}

struct CompiledTemplateSet {
    templates: Vec<(String, CompiledTemplate)>,
}

impl CompiledTemplateSet {
    fn replace(&self, text: &mut String) {
        for (name, template) in self.templates.iter() {
            println!("       # Replace {}", name);
            template.replace(text);
        }
    }
}

struct UpdateFile {
    path: String,
    content: String,
}

impl UpdateFile {
    fn new<T: convert::Into<String>>(path: T) -> Result<UpdateFile> {
        let path = path.into();
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Unable to open {} to update content", path))?;
        Ok(UpdateFile {
            path,
            content,
        })
    }

    fn update(&mut self, templates: &CompiledTemplateSet) {
        templates.replace(&mut self.content);
    }

    fn save(&self) -> Result<()> {
        fs::write(&self.path, &self.content)
            .with_context(|| format!("Unable to save {} with updated content", self.path))
    }
}

#[derive(Default,)]
struct LinkUpdater {
    filenames: Vec<String>,
    templates: TemplateSet,
    generated_files: HashMap<String, UpdateFile>,
}

impl LinkUpdater {
    fn filenames_into(&self) -> Vec<String> {
        self.filenames.clone()
    }

    fn filename<T: convert::Into<String>>(mut self, filename: T) -> Self {
        self.filenames.push(filename.into());
        self
    }
    fn template<N: convert::Into<String>, T: convert::Into<Template>>(mut self, name: N, template: T, line_bounds: Matching) -> Self {
        self.templates.add(name, template, line_bounds);
        self
    }

    fn load_one(&mut self, filename: String) -> Result<&mut UpdateFile> {
        if self.generated_files.contains_key(&filename) {
            Ok(self.generated_files.get_mut(&filename).unwrap())
        } else {
            Ok(
                self.generated_files
                    .entry(filename.to_owned())
                    .or_insert(UpdateFile::new(filename.to_owned())?)
            )
        }
    }

    fn update_all(&mut self, templates: CompiledTemplateSet) -> Result<()> {
        self.generated_files = HashMap::with_capacity(self.filenames.len());
        for filename in self.filenames_into() {
            println!("    * Update file {}", filename);
            let file = self.load_one(filename)?;
            file.update(&templates);
        }
        Ok(())
    }

    fn generate_all(&mut self, source: &Version, target: &Version) -> Result<()> {
        self.update_all(self.templates.compile(source, target))
    }

    fn revert_all(&mut self, source: &Version, target: &Version) -> Result<()> {
        self.update_all(self.templates.reverse().compile(target, source))
    }

    fn save_all(&self) -> Result<()> {
        for file in self.generated_files.values() {
            file.save()?;
        }
        Ok(())
    }

    fn process_all(&mut self, target: &Version) -> Result<()> {
        let source = Version::new("main");
        self.generate_all(&source, target)?;
        self.save_all()
    }

    fn restore_all(&mut self, source: &Version) -> Result<()> {
        let target = Version::new("main");
        self.revert_all(source, &target)?;
        self.save_all()
    }
}

struct ChangelogUpdater {
    file: UpdateFile,
}

impl ChangelogUpdater {
    fn load() -> Result<Self> {
        let file = UpdateFile::new("./CHANGELOG.md")
            .context("Loading changelog")?;
        Ok(Self {
            file,
        })
    }

    fn path(&self) -> &String {
        &self.file.path
    }

    fn process(&mut self, target: &Version) -> Result<()> {
        let templates = TemplateSet::default()
            .register(
                "Unreleased -> Version",
                (
                    "## [Unreleased]",
                    format!("## [Unreleased]\n\n## [{{version}}] - {date}\n\n* **Crate**: https://crates.io/crates/macon/{{version}}\n* **Documentation**: https://docs.rs/macon/{{version}}/macon/", date = chrono::Utc::now().date_naive()),
                ),
                Matching::Line,
            );
        let compiled = templates.compile(&Version::new("Unreleased"), target);
        self.file.update(&compiled);
        self.file.save()
            .context("Update changelog")
    }
}

struct CargoUpdater {
    version: Version,
    file: UpdateFile,
}

impl CargoUpdater {
    fn load() -> Result<Self> {
        let file = UpdateFile::new("./Cargo.toml")
            .context("Unable to read Cargo manifest")?;
        let manifest = &file.content;
        let start_tag = "\nversion = \"";
        if let Some(start) = manifest.find(start_tag) {
            let start = start + start_tag.len();
            if let Some(end) = manifest[start..].find("\"") {
                let end = start + end;
                let version = Version::new(&manifest[start..end]);
                Ok(CargoUpdater {
                    version,
                    file,
                })
            } else {
                bail!("Unable to locate version end in Cargo manifest");
            }
        } else {
            bail!("Unable to locate version start in Cargo manifest");
        }
    }

    fn path(&self) -> &String {
        &self.file.path
    }

    fn process(&mut self, target: &Version) -> Result<()> {
        let templates = TemplateSet::default()
            .register("version", "version = \"{version}\"", Matching::Line)
            .register("macon_derive", "macon_derive = { version = \"={version}\", path = \"macon_derive\" }", Matching::Line)
            .register("macon_api", "macon_api = { version = \"={version}\", path = \"macon_api\" }", Matching::Line);
        let compiled = templates.compile(&self.version, target);
        self.file.update(&compiled);
        self.file.save()
            .context("Update Cargo.toml")
    }
}

pub fn main() -> Result<()> {
    let mut args = std::env::args();
    if args.len() != 2 {
        bail!("Expecting exactly 1 arguments: released version");
    }
    args.next(); // Skip executable
    let raw_release = args.next().unwrap();

    let release_version = semver::Version::parse(&raw_release)
        .with_context(|| format!("Provided release {} isn't semver compliant", raw_release))?;

    let mut next_version = release_version.clone();
    next_version.pre = semver::Prerelease::EMPTY;
    next_version.build = semver::BuildMetadata::EMPTY;
    next_version.patch += 1;

    let release = release_version.into();
    let next = next_version.into();


    println!(" > Check git status");
    let git = Git::new();
    if ! git.is_clean()? {
        bail!("Git working directory is not clean");
    }

    println!(" > Update links into files");

    let mut github = LinkUpdater::default()
        .filename("./CRATES_IO.md")
        .filename("./src/lib.rs")
        .template("Github folder", "https://github.com/loganmzz/macon-rs/tree/{version}/", Matching::Exact)
        .template("Github file", "https://github.com/loganmzz/macon-rs/blob/{version}/", Matching::Exact)
    ;
    let mut crate_io = LinkUpdater::default()
        .filename("./macon_api/CRATES_IO.md")
        .filename("./macon_api/src/lib.rs")
        .filename("./macon_derive/CRATES_IO.md")
        .filename("./macon_derive/src/lib.rs")
        .template("Crates.io", ("(https://crates.io/crates/macon)", "(https://crates.io/crates/macon/{version})"), Matching::Exact)
    ;
    github.process_all(&release)
        .context("Processing Github links")?;
    crate_io.process_all(&release)
        .context("Processing Crates.io links")?;

    println!(" > Release Cargo.toml");
    let mut cargo_manifest = CargoUpdater::load()?;
    println!("    * Current: '{}'", cargo_manifest.version);
    cargo_manifest.process(&release)
        .context("Releasing Cargo.toml")?;

    println!(" > Release CHANGELOG.md");
    let mut changelog = ChangelogUpdater::load()?;
    changelog.process(&release)
        .context("Releasing CHANGELOG.md")?;


    println!(" > Git stage updated files");
    let mut filenames = Vec::new();
    filenames.extend(github.filenames_into());
    filenames.extend(crate_io.filenames_into());
    filenames.extend([cargo_manifest.path().to_owned()]);
    filenames.extend([changelog.path().to_owned()]);
    git.add(&filenames)?;


    println!(" > Restore links into files");
    github.restore_all(&release)
        .context("Restoring Github links")?;
    crate_io.restore_all(&release)
        .context("Restoring Crates.io links")?;

    println!(" > Post-Release Cargo.toml");
    let mut cargo_manifest = CargoUpdater::load()?;
    println!("    * Current: '{}'", cargo_manifest.version);
    cargo_manifest.process(&next)
        .context("Post-releasing Cargo.toml")?;

    println!(" > Git commit released files");
    git.commit(format!("Release {}", release))
        .context("Unable to create relase commit")?;

    println!(" > Git tag release");
    git.tag(release.to_string())
        .context("Unablt to create release tag")?;

    println!(" > Git stage post-release files");
    let mut filenames = Vec::new();
    filenames.extend(github.filenames_into());
    filenames.extend(crate_io.filenames_into());
    filenames.extend([cargo_manifest.path().to_owned()]);
    git.add(&filenames)?;

    println!(" > Git commit post-release files");
    git.commit(format!("Prepare next release {}", next))
        .context("Unable to create relase commit")?;

    println!("

Pre-release process has been success!
Now:

* Publish crates from tag:

git checkout {release} &&
cargo publish --package macon_api &&
cargo publish --package macon_derive &&
cargo publish --package macon

* Push tag and main branch:

git push origin main {release}
",
    release = release,
);

    Ok(())
}
