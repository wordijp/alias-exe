# alias-exe

This is a provides Bash alias-like to Windows.  
It supports Bash-like arguments `$1, "$@", etc`, nested command `$( ... )`, and mruby as a unique feature.

## Download

See [releases](https://github.com/wordijp/alias-exe/releases)

## Install

Support [scoop.sh](https://scoop.sh/), you can install alias-exe from [wordijp/scoop-bucket](https://github.com/wordijp/scoop-bucket).

```cmd
> scoop bucket add wordijp https://github.com/wordijp/scoop-bucket
> scoop install wordijp/alias-exe
```

## How does this work?

When you execute `alias.exe` directly, it manage command the alias list.  
Please check the help for details.

```cmd
> alias help
```

When executed a symbolic link created by `alias edit <alias_name>`, it executes the command and mruby code.  
The execution contents are saved as a txt file in the same directory as the symbolic link.

```cmd
> alias edit hello
```

```
# hello.txt
echo hello $(echo world)
```

```
.
├── alias.exe
└── list
    ├── hello.exe -> ../alias.exe
    └── hello.txt
```

## Usage

See [document](http://wordijp.github.io/alias-exe/) for details.  
*offline file is `docs/index.html`*

There is some Support for

- run command
- nested command
    - `$( ... )`
- arguments
	- bash-like arguments
	    - `$0`
			- alias name
	    - `$1`, `$2`, `$3`, ..., `$9`
	    - `$#`
	    - `"$@"`
	- unique arguments
	    - `"$+"`
	        - Expands arguments on a single line, like `"$*"`, but keeps asterisks it.
	        - ex) `echo "$+"`, run `> test_alias "ab cd" fo* bar`, expands arguments is `"ab cd fo* bar"`
- built-in commands
    - @set \<key\>=\<value\>
        - Set environment variable(command prompt-like)
    - @pushd \<path\>
        - Save and then change the current directory(bash-like)
    - @popd
        - Restore the top entry from the directory stack(bash-like)
- mruby as glue code
````
```ruby
code
```
````  

- nested mruby(ERB-like)
    - `<%= ... %>`

## Examples

### Basic

Create `hello` alias:

```cmd
> alias edit hello
```

edit:

```
# hello.txt
echo hello $(echo world)
```

run:

```cmd
> hello
hello world
```

### Others

<details>
<summary>mruby example</summary>

```cmd
> alias edit mruby-example
```
````
# mruby-example.txt
```ruby
puts 'from mruby'
def say
  "hello world"
end

ary = [1, 'mruby array', 2, 'to', 'cmd', 3.14]
```

echo <%= say %>
echo <%= ary.to_cmd %>
````

```cmd
> mruby-example
from mruby
"hello world"
1 "mruby array" 2 to cmd 3.14
```

</details>

<details>
<summary>Nested commmand example</summary>

```cmd
> alias edit nested-command-example
```

````
# nested-command-example.txt
```ruby
underscore = "_"
hello_world = "success!"
```

# $1: hello, $2: world
# need sed command installed, for example, msys2
echo <%= $(echo $(echo $1)<%= underscore %>XXX | sed 's/XXX/$(echo $2)/g') %>
````

```cmd
> nested-command-example hello world
success!
```

</details>

<details>
<summary>FizzBuzz example</summary>

*Implement it in a weird way :)*


```cmd
> alias edit rng
```

````
# rng.txt
```ruby
for i in $1..$2 do
  puts i
end
```
````

```cmd
> alias edit fizzbuzz
```

````
# fizzbuzz.txt
```ruby
while s = STDIN.gets
  n = s.to_i
  if n > 0 && n % 15 == 0
    puts "FizzBuzz"
  else
    puts s.chomp
  end
end
```
````

```cmd
> alias edit fizz
```

````
# fizz.txt
```ruby
while s = STDIN.gets
  n = s.to_i
  if n > 0 && n % 3 == 0
    puts "Fizz"
  else
    puts s.chomp
  end
end
```
````

```cmd
> alias edit buzz
```

````
# buzz.txt
```ruby
while s = STDIN.gets
  n = s.to_i
  if n > 0 && n % 5 == 0
    puts "Buzz"
  else
    puts s.chomp
  end
end
```
````

Finally, run

```cmd
> rng 1 100 | fizzbuzz | fizz | buzz
1
2
Fizz
4
Buzz
Fizz
...
97
98
Fizz
Buzz
```

</details>

## License

MIT
