use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html!(<>
        <h1>{"Hey Hey"}</h1>
        <script type="module">
         { r#"  // Import the functions you need from the SDKs you need
            import { initializeApp } from "https://www.gstatic.com/firebasejs/9.21.0/firebase-app.js";
            import { getAnalytics } from "https://www.gstatic.com/firebasejs/9.21.0/firebase-analytics.js";
            // TODO: Add SDKs for Firebase products that you want to use
            // https://firebase.google.com/docs/web/setup#available-libraries
          
            // Your web app's Firebase configuration
            // For Firebase JS SDK v7.20.0 and later, measurementId is optional
            const firebaseConfig = {
              apiKey: "AIzaSyD896Z6s9coanwsjpBsuwK9VxTDbUyXIIE",
              authDomain: "test-b6ab5.firebaseapp.com",
              projectId: "test-b6ab5",
              storageBucket: "test-b6ab5.appspot.com",
              messagingSenderId: "269229307102",
              appId: "1:269229307102:web:2af1595d018d337c78be4f",
              measurementId: "G-MPV4HJCYRS"
            };
          
            // Initialize Firebase
            const app = initializeApp(firebaseConfig);
            const analytics = getAnalytics(app);"#}
          </script>
        </>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
