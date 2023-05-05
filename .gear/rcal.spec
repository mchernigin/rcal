Name: rcal
Version: 0.1
Release: alt1

Summary: Replica of Unix `cal` command
License: GPLv2+
Group: Other

BuildRequires: rust-cargo

Source0: %name-%version.tar

%description
Displays a simple calendar. Just like cal.

%prep
%setup -q
mkdir -p .cargo
cat >> .cargo/config <<EOF
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

%build 
cargo build --offline --release

%install
mkdir -p %buildroot%_bindir
install -Dm0755 target/release/%name %buildroot%_bindir/

%files
%_bindir/%name
