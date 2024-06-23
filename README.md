# GitHub GraphQL API with Rust

This repository contains a Rust project that utilizes GraphQL to interact with GitHub issues. It showcases how to perform various operations such as querying and mutating GitHub issues using Rust as the programming language and GraphQL for communication with the GitHub API.

The project is structured to include a GraphQL server setup with Rocket, a web framework for Rust, and `async-graphql`, a GraphQL server library for Rust. The server allows querying GitHub issues and performing mutations like creating, updating, and deleting issues.

Key components of the project include:

- **GraphQL Schema Definitions**: Located in `github_issue/src/schema/`, these files define the GraphQL schema for GitHub issues, including types and mutations.
- **Query and Mutation Handlers**: Implemented in `github_issue/src/handler/`, these handlers process GraphQL queries and mutations, interacting with the GitHub API.
- **Rocket Server Setup**: The `github_issue/src/bin/main.rs` file sets up the Rocket server and routes for handling GraphQL queries and mutations.

This project serves as an example of how Rust and GraphQL can be used together to build a backend service that interacts with GitHub issues, demonstrating the power and flexibility of Rust in web development and the efficiency of GraphQL for API communication.

## Prerequisites

Before running the project, ensure you have the following prerequisites:

- Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).
- A GitHub personal access token with the necessary permissions to interact with issues. You can create a personal access token in your GitHub account settings.
- A GitHub repository with issues that you want to interact with using the GraphQL API.
- A text editor or IDE for writing and editing Rust code.
- Basic knowledge of Rust programming and GraphQL concepts.
- Familiarity with using the command line to run Rust projects.
- An internet connection to communicate with the GitHub API.
- Optional: A tool like [Hoppscotch](https://hoppscotch.io/graphql) or [GraphQL Playground](https://studio.apollographql.com/sandbox/explorer) for testing GraphQL queries and mutations.


## Getting Started

To get started with the project, follow these steps:

1. Clone the repository to your local machine using Git:

   ```bash
   git clone https://github.com/AkshitVadodariya1201/GitHub-GraphQL-APIs-with-Rust
    ```
2. Navigate to the project directory:
3. Create a `.env` file in the project root directory and add your GitHub personal access token:

   ```plaintext
   GITHUB_TOKEN=your_personal_access_token_here
   ```

4. Build and run the project using Cargo:

   ```bash
   cargo run
   ```
5. Once the server is running, you can access the GraphQL playground at `http://localhost:8000` to test queries and mutations.
6. Use the GraphQL playground to interact with GitHub issues, query issue data, create new issues, update existing issues, and delete issues.
7. Explore the project structure, code, and GraphQL schema definitions to understand how the project is implemented and how Rust and GraphQL are used together.
8. Experiment with different queries and mutations to interact with GitHub issues and see the results in real-time.
9.  Modify the project to suit your requirements, add new features, or integrate additional functionality using Rust and GraphQL.
10. Enjoy exploring the world of Rust and GraphQL with GitHub issues!

## Resources

- [Rust Programming Language](https://www.rust-lang.org/): Official website for the Rust programming language, with documentation, tutorials, and resources.
- [GraphQL](https://graphql.org/): Official website for GraphQL, with guides, specifications, and resources for learning GraphQL.
- [GitHub GraphQL API](https://docs.github.com/en/graphql): GitHub GraphQL API documentation, including guides, reference materials, and examples.
- [Rocket](https://rocket.rs/): Official website for the Rocket web framework for Rust, with documentation, guides, and examples.
- [async-graphql](https://async-graphql.github.io/async-graphql/): Official website for the `async-graphql` library, a GraphQL server library for Rust, with documentation, guides, and examples.
- [GitHub Personal Access Tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token): GitHub documentation on creating personal access tokens for API authentication.
- [Hoppscotch](https://hoppscotch.io/graphql): Online GraphQL IDE for testing queries and mutations against a GraphQL endpoint.
- [GraphQL Playground](https://studio.apollographql.com/sandbox/explorer): Online GraphQL IDE for exploring and testing GraphQL APIs.
- [Rustup](https://rustup.rs/): Official website for `rustup`, the Rust toolchain installer, with installation instructions and guides.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

Happy coding!🦀🚀