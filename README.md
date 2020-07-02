# qj

Dump JSON safely from command-line

## Usage

`qj` do the reverse of `jq`.

```bash
   qj -e .=3
3

   qj -e .x=1 -e .y=2 -e .z[0]=3
{"x":1,"y":2,"z":[3]}

   qj -e '.hello="world"'  # Use quoting for Shell Escaping
{"hello":"world"}

   qj -e '.persons[1].name="Alice"'
{"persons":[null,{"name":"Alice"}]}
```

### Invalid values are Strings.

```bash
   qj -e '.=hoge'
"hoge"

   qj -e '.=ho"ge'
"ho\"ge"
```
