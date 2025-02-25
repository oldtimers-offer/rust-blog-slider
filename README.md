### Blog Slider Project (Full Stack)

This project is a dynamic blog slider application developed in Rust, utilizing the Actix Web framework for the backend and Yew for the frontend. The backend employs Actix Web (version 4.9.0) to efficiently handle HTTP requests, while Actix CORS (version 0.7.0) is implemented to manage cross-origin requests, ensuring smooth integration with the frontend. Data serialization and deserialization are managed by Serde (version 1.0.218).

On the frontend, Yew (version 0.21.0) is used with the client-side rendering (CSR) feature to create a responsive and interactive user interface. The application makes HTTP requests to the backend using Reqwest (version 0.12.12) with JSON support for fetching blog data. Asynchronous operations are handled with wasm-bindgen-futures (version 0.4.50), providing a seamless user experience. The blog slider displays posts in a visually appealing format, allowing users to navigate through different entries effortlessly.
