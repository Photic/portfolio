# Portfolio App, using Rust, Actix-web, Handlebars and HTMX

30 - Sep 2023 : 13:45. Made the repository public

## Content

- [Portfolio App, using Rust, Actix-web, Handlebars and HTMX](#portfolio-app-using-rust-actix-web-handlebars-and-htmx)
  - [Content](#content)
  - [Project](#project)
  - [Conditional rendering](#conditional-rendering)
    - [Problem](#problem)
    - [Solution](#solution)

## Project

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

Provided that you have the required dependencies installed, ei Rust, Node, NPM and Cargo with cargo-watch plus the tailwindcss cli globally. I will not go into details on how to install these, as there are plenty of guides out there on how to do it.

## Conditional rendering

Because I am not going to use a frontend framwork, Iv opted to do some conditional_rendering of my html code. This was nessecary because of how handlebars and HTMX works. The problem statement is like this.

### Problem

If I am at:

```bash
http://localhost:8090
```

I want to render the layout.html template, and add the home.html template to the body of the layout.html template. This is easy just using handlebars.

```html
{{> home }}
```

However, what if I change the url to:

```bash
http://localhost:8090/about
```

Well, a way to handle this would be to have a route in actix_web for every single page, and then just render the layout.html template, and add the about.html.
However, now I have send the entire layout.html file again to the browser. This is not ideal, as I would like to only send the about.html part, and have it switch out in the browser where it is needed. This is where HTMX comes in.

```html
<button 
    hx-get="about" 
    hx-boost="true" 
    hx-push-url="true" 
    hx-trigger="click" 
    hx-target="#content" 
    hx-swap="innerHTML">
    About
</button>
```

This code, asks on the endpont /about for the html code, and then replaces the innerHTML of the element with the id of content with the html code it gets back. This is great, because now I can just have one route in actix_web. However, what if I refresh the browser on /about. About will only render when when the button is clicked. Because that is the only way to code knows how to get me my code.

So, how do we solve this. We could do an entire if else if statement in our layout.html file to check what the url is, and then render the correct template. But that is not very nice, and it would be a lot of code. Plus I chose to do HTMX to reduce this kind of frontend code, and to get rid of as much javascript as possible.

### Solution

In comes my solution, in the file below, I have created a Rust function amptly called conditional_render. This function looks at the header of the request. If the header contains the key "hx-target", then we know that HTMX is trying to get some code that it can replace in some target, and that we only have to send the partial template back to the frontend. If the header does not contain the key "hx-target", then we know that a full load is required, and we can render the entire layout.html with the requested partial template inserted into the layout content.

[Link to app.rs](./src/app.rs)

On top of this, we use an actix_web service to determine the current page_name (partial) which we can pass on to handlebars when a full load is required. This way we wont need to create a new route for every new file we create, instead it is determined by the url.

```rust
.service(
    web::resource("/{page_name}").route(web::get().to(app::default_page_navigation)),
    )
```

Here, page_name is just send directly to our conditional_render function. Which will give us the page we need on a full load. If there is no page to render, we the the NOT_FOUND page.
