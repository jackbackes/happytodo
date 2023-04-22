# 🚀🧠✅ HappyTodo-RocketGPT-Plugin  ✅🧠🚀

Welcome to the **ChatGPT Todo Plugin**! This project is all about helping you manage your todos with ease and style, right within your ChatGPT conversations. Say goodbye to the hassle of switching between multiple apps to keep track of your tasks, and hello to an efficient and seamless experience! 🎉

## 🌟 Features

- Create, update, and delete todos for different users
- Fetch todos for a specific user
- Fetch all todos
- Health check endpoint
- OpenAPI Specification (YAML)
- Plugin manifest and logo

## 📚 How it works

This project is built using the [Rocket](https://rocket.rs) web framework for Rust, and uses an in-memory HashMap to store todos. The todos are organized per user, and you can perform the basic CRUD operations on them through various HTTP endpoints.

## 🚦 Getting Started

To get started, follow these steps:

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
2. Clone this repository:

   ```bash
   git clone https://github.com/yourusername/chatgpt-todo-plugin.git
   ```

3. Change to the project directory:

   ```bash
   cd chatgpt-todo-plugin
   ```

4. Build and run the project:

   ```bash
   just run
   ```

Now, the Todo Plugin server should be up and running at `http://localhost:8000`.

## 🔧 Configuration

To configure the allowed origins for CORS, update the `AllowedOrigins::some_exact` array with your desired domains in the `main()` function. The default configuration allows localhost and the OpenAI Chat domain.

## 📖 API Endpoints

The following API endpoints are available:

- `GET /hello`: Health check
- `GET /todos`: Fetch all todos
- `GET /todos/<username>`: Fetch todos for a specific user
- `POST /todos/<username>`: Add a todo for a specific user
- `DELETE /todos/<username>`: Delete a todo for a specific user
- `GET /logo.png`: Fetch the plugin logo
- `GET /.well-known/ai-plugin.json`: Fetch the plugin manifest
- `GET /openapi.yaml`: Fetch the OpenAPI specification

## 🛠️ Customization

Feel free to customize the project to fit your needs! You can:

- Add new endpoints
- Change the data storage method (e.g., use a database instead of an in-memory HashMap)
- Customize the plugin logo and manifest
- Tweak the OpenAPI specification

Happy coding and enjoy your new and improved ChatGPT Todo experience! 🎊🎉
