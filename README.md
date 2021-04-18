# `dahe`

`dahe` is a cli tool for signing [EIP-3074](https://eips.ethereum.org/EIPS/eip-3074) `auth` messages.

## Usage

```console
$ dahe import pk 0x45a915e4d060149eb4365960e6a7a45f334393093061116b197e3240065ff2d8
importing 0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b
Password:
Password (again):

$ dahe ls
Available keys
---
0:      0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b

$ dahe sign 0x52bc44d5378309ee2abf1539bf71de1b7d7be3b5 01 01 01
commit preimage: 010101
commit: 7ad37e9ae69046be83354f8de5e8b4814d21075a11ce84f5e52f89733145e87c
sig preimage: 0300000000000000000000000052bc44d5378309ee2abf1539bf71de1b7d7be3b57ad37e9ae69046be83354f8de5e8b4814d21075a11ce84f5e52f89733145e87c
v: 0
r: 407d189a49709eb86c0adc4c229564bb3abc74fc1426cc3070e50f01c9aa2667
s: 2ca43ac802374b451461bfc7e5ce924de60d42846cd81c7fe5525276bf923635
```
