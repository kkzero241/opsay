# Opsay

```
now loading
opsay mission
operation:
o.p.s.a.y.

obviously
pretty
silly
app,
y'know?
```

This is **opsay**, a cli program that takes input text and spits it out as an acronym.

The intent is to mimic the acronyms used for [the episode titles of Codename: Kids Next Door](https://en.wikipedia.org/wiki/List_of_Codename:_Kids_Next_Door_episodes).

# Usage

`opsay [options] <text>`

Optional arguments:
* `-l`: Lessen the output to just the org name and acronym.
* `-n <name>`: Change the org name from the default "opsay". Make sure to use quotes if `<name>` contains whitespace.

# Musings

I had quite a bit of fun developing this. I've always loved Kids Next Door since I was a kid, had a soft spot for [cowsay](https://en.wikipedia.org/wiki/Cowsay) since I was a teen, and have wanted to try coding in Rust ever since I found out about it. So, once I suddenly got the idea for a cowsay-and-KND-inspired toy program, I got started. I'm more than a bit amazed at just how perfect Rust ended up being for this task, with its breadth of string support powered by the magic of Unicode. The only runtime panics I encountered were a result of the string unwrapping initially not accounting for an empty vector, and I got those ironed out quite quickly. Thus, I hope this program, even for how goofy and small-scale it is, is able to make someone's day.