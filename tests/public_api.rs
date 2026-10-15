use deno_graph::packages::resolve_version;
use deno_graph::source::MemoryFileSystem;
use deno_graph::source::MemoryLoader;
use deno_graph::MediaType;
use url::Url;

#[test]
fn resolves_the_highest_matching_package_version() {
  let requirement = deno_semver::VersionReq::parse_from_npm("^1.0.0").unwrap();
  let versions = [
    deno_semver::Version::parse_standard("1.0.0").unwrap(),
    deno_semver::Version::parse_standard("1.4.0").unwrap(),
    deno_semver::Version::parse_standard("2.0.0").unwrap(),
  ];
  assert_eq!(
    resolve_version(&requirement, versions.iter()),
    Some(&versions[1])
  );
}

#[test]
fn public_source_and_media_type_exports_remain_available() {
  let specifier = Url::parse("file:///mod.ts").unwrap();
  assert_eq!(MediaType::from_specifier(&specifier), MediaType::TypeScript);
  let _loader = MemoryLoader::default();
  let _filesystem = MemoryFileSystem::default();
}
