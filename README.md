# What is It?
A CLI App with which you can download files from R34 by tags.

I would not recommend using it but hey have fun.

I made this little Programm with my first and only functional library? (im not sure if it can be called one).
It is really bad but if you want to try it you can find it here <https://crates.io/crates/r34_api>

# How to use it 
As soon as you run the exe file it will give you instructions.

## Tags
`Add Tags. Seperate with spaces.`

You can then write any tags which would work on r34's page like 'tits' 'penis'.
Btw. you can Blacklist Tags by adding a '-' before the tag, like '-yuri'

If you dont set any tags you get asked one more time(I will make it ask till it gets some input) to add tags,
and give some hints on what you can add.

```
Please add some Tags.
Seperate with spaces e.g. 'big_ass cock' or '-yaoi -tits' for blacklisting tags
```

## Limit
After you have set your Tags you can choose a limit of Posts you want to retrieve.

❗R34's API only allows 1000 Posts per Request. 

`Add Response Limit. Max 1000.`

If you dont set a limit or your input couldnt be read it will quit after 5 seconds.

```
Couldn't read input
U probably didn't enter any number.
Or it could be a to big input.
Please try again with another input
```
