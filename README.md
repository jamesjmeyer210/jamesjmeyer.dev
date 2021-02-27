# jamesjmeyer.dev

## Uri Structure

```shell
/               -> index_controller.rs
/resume         -> resume_controller.rs
/contact        -> contact_controller.rs
/blog           -> blog_controller.rs
  /{blog_index} -> blog_controller.rs
```

## TODO

 - [ ] Provide Markdown to HTML server side rendering for blog posts.
 - [ ] Provide a contact form on the contact page.
    - [ ] Dispatch an email or text message when someone uses the contact form.
    - [ ] Use a queue to rate limit the number of times a text or email is sent.
 - [ ] Get rid of all javascript files.
 - [ ] Provide a con-job / shell script for easy, regular re-deployment.
 - [ ] Create a tagging system for organizing `#tags`.