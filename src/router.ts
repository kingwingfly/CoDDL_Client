import { createRouter, createWebHashHistory } from "vue-router";
import Welcome from "./components/Welcome.vue"
import SignUp from "./components/SignUp.vue"
import User from "./components/User.vue";
import Group from "./components/Group.vue";
import NotFound from "./components/NotFound.vue";

const routes = [
    { path: '/', name: "Welcome", component: Welcome },
    { path: '/sign_up/:name', name: "SignUp", component: SignUp, props: true },
    { path: '/sign_up', name: "SignUp", component: SignUp },
    { path: '/user/:name/:group', name: "Group", component: Group, props: true },
    { path: '/user/:name', name: "User", component: User, props: true },
    { path: '/:pathMatch(.*)*', name: "NotFound", component: NotFound, props: true },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes: routes,
})

export default router