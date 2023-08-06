CC := rustc

#[unix]
default:
  {{ CC }} thunderlord.rs
  ./thunderlord

#[windows]
default:
  {{ CC }} thunderlord.rs
  .\thunderlord.exe
