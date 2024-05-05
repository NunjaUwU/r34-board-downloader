# What is It?
A CLI App with which you can download files from R34 by tags.

I would not recommend using it but hey have fun.

I made this little Programm with my first and only functional library? (im not sure if it can be called one).
It is really bad but if you want to try it you can find it here <https://crates.io/crates/r34_api>

# How to use it 
Because its my First programm and im to lazy and stoopid to add a Code Signing Certificate you have to whitelist the exe file in your Antivirus.
I would recommend making a new folder where you execute it cause its going to create some extra files.
As soon as you run the exe file it will give you instructions.

## Tags
`Add Tags. Seperate with spaces.`

Inside the config.txt file you can write any tags which would work on r34's page like 'tits' 'penis'.
Btw. you can Blacklist Tags by adding a '-' before the tag, like '-dick'.

If you don't add any tags it will download the newest posts from the index page.

## Limit
When you have set some Tags and started the Program you can choose a limit of Posts you want to retrieve.

❗R34's API only allows 1000 Posts per Request. 

`Add Response Limit. Max 1000.`

If you dont set a limit or your input couldnt be read it will quit after 5 seconds.

```
Couldn't read input
U probably didn't enter any number.
Or it could be a to big input.
Please try again with another input
```

## Request Amount
After setting the limit you have to set a request amount. Each Request will request the amount of posts you
have set with the limit.

If you have a limit of 1000 and a request amount of 2 it will download 2000 Posts in parallel.
It works like the pages on R34s website, but with 1000 posts per page.

## Downloading Posts
There is like no feedback how many it has downloaded so you just have to wait.

If it fails it will stop.

## Errors
If no download started you should check if
- You have set the tags right
- Your antivirus hasnt blocked the process