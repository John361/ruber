# Ruber
[![Ruber build](https://github.com/John361/ruber/actions/workflows/build.yml/badge.svg)](https://github.com/John361/ruber/actions/workflows/build.yml)
[![Ruber release](https://github.com/John361/ruber/actions/workflows/release.yml/badge.svg)](https://github.com/John361/ruber/actions/workflows/release.yml)

## Dev help for folders creation
```shell
rm -rf /tmp/ruber
mkdir -p /tmp/ruber
mkdir -p /tmp/ruber/source1
mkdir -p /tmp/ruber/destination1
mkdir -p /tmp/ruber/source2
mkdir -p /tmp/ruber/destination2
mkdir -p /tmp/ruber/source3
mkdir -p /tmp/ruber/destination3
mkdir -p /tmp/ruber/archives

mkdir -p /tmp/ruber/keys
touch /tmp/ruber/keys/user

echo "hello world file1" > /tmp/ruber/source1/file1.txt
echo "hello world file2" > /tmp/ruber/source2/file2.txt
echo "hello world file3" > /tmp/ruber/source3/file3.txt
```
