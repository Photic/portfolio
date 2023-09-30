# Portfolio App, using Rust, Actix-web, Handlebars and HTMX

30 - Sep 2023 : 13:45. Made the repository public

I started this project because I wanted to be better at Rust, but also because the world of "hypermedia as the engine of application state" (HATEOAS) is something I want to explore. In my opinion, the best way to learn something is to build something with it. So, I decided to build a portfolio app, using Rust, Actix-web, Handlebars and HTMX.

- Rust, because I like the typesafety and the error handling of the rust-analyzer. Being albe to see where in your code something is wrong, and a lot of the time even getting suggestions on how to fix said code is a great help.
- Actix-web was chosen becasue I already had some prioer experience with it, and I like the way it works. Especially with the macros for defining routes. Reminds me of how I use to do it at the university with annotations in Java, and is also how TSOA works, which I have spend some time with.
- Handlebars seemed like a good choice for templating my html, I have never used it before, I tried jinja like engines, but did ultimately not like the syntax all that much. I wanted something that was easy to read, and easy to write. Handlebars seemed to fit the bill.
- HTMX is a new thing for me, I have never used it before, but I have been looking at it for a while. I like the idea of being able to write html, and then just add some attributes to it, and then it just works.

For styling iv just gone the simple route of adding TailwindCSS, and then just using the classes it provides.

To get it all running together iv made a run.sh file, which just runs the cargo --watch command, and in a seperate but connected terminal, runs the tailwindcss npx watch command. So starting the project is a simple as.

```bash
sh run.sh
```

## Content

- [Portfolio App, using Rust, Actix-web, Handlebars and HTMX](#portfolio-app-using-rust-actix-web-handlebars-and-htmx)
  - [Content](#content)
