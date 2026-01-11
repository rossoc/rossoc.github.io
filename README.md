# Carlo Rosso's Personal Website

This repository contains the source code for my personal website, which serves as my portfolio and personal knowledge base.

You can visit the live version at [rossoc.github.io](https://rossoc.github.io).

## Built with [Scripta](https://github.com/rossoc/Scripta)

This website is powered by **Scripta**, a custom-built static site generator written in **Rust**. 

### Key Features of Scripta:
- **Markdown-First**: Content is written in Markdown and automatically converted to high-performance HTML.
- **Rust-Powered**: Fast and reliable build process leveraging the Rust ecosystem.
- **Automated Workflow**: Integrated with GitHub Actions for seamless deployment to GitHub Pages.
- **Extensible Layouts**: Uses a flexible templating system (located in the `layout/` directory) to maintain a consistent look and feel across the site.

## Repository Structure

- `notes/`: Markdown files for blog posts and technical notes.
- `portfolio/`: Details about projects, experience, and my CV.
- `preview/`: Content for presentations.
- `assets/`: Static assets including CSS, JavaScript, fonts, and images.
- `layout/`: HTML templates used by Scripta to build the final site.
- `.github/workflows/`: CI/CD configuration for automated building and deployment.
