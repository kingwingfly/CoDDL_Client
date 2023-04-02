import { createRouter, createWebHashHistory } from "vue-router";
import Welcome from "./components/Welcome.vue"
import SignUp from "./components/SignUp.vue"
import User from "./components/User.vue";

const routes = [
    { path: '/', name: "Welcome", component: Welcome },
    { path: '/sign_up/:name', name: "SignUp", component: SignUp, props: true },
    { path: '/sign_up', name: "SignUp", component: SignUp },
    { path: '/user/:name', name: "User", component: User, props: true },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes: routes,
})

export default router