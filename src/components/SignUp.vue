<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { confirm } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import { useRoute } from "vue-router";


const router = useRouter();
const route = useRoute();
const username = ref(route.params.name);
const password = ref();
const ensure_password = ref();
const email = ref("");
const sign_up_info = ref("");

async function sign_up() {
    let sign_up_res = false;
    // if no input, do nothing
    if (username.value == null || password.value == null || ensure_password.value == null) {
        sign_up_info.value = "Some info missed";
        return
    }
    if (password.value != ensure_password.value) {
        sign_up_info.value = "The two inputs are different.";
        ensure_password.value = null;
        return
    }
    // ask user to confirm signing up
    const whether_sign_up = await confirm('You are going to sign up for ' + username.value + ' with password ' + password.value + 'and email' + email.value, { title: 'Confirm?', type: 'warning' });
    if (whether_sign_up) {
        sign_up_res = await invoke("sign_up", { username: username.value, password: password.value, email: email.value });
    } else {
        sign_up_info.value = "Sign up cancelled.";
        return
    }
    if (sign_up_res) {
        // if succeed, then jump to user page
        sign_up_info.value = "Succeed signing up.";
        router.push({ path: `/user/${username.value}` });
    } else {
        sign_up_info.value = "Faild signing up, user exists.";
    }
}
</script>

<template>
    <h2>Essential</h2>
    <div class="card">
        <input id="username" v-model="username" placeholder="Enter your username..." />
        <input id="password" v-model="password" placeholder="Enter your passwold..." />
        <input id="ensure_password" v-model="ensure_password" placeholder="Ensure your passwold..."
            @keyup.enter="sign_up()" />
    </div>
    <h2>Optional</h2>
    <div>
        <input id="email" v-model="email" placeholder="Enter your email..." @keyup.enter="sign_up()" />
    </div>
    <div>
        <button id="sign_up" type="button" @click="sign_up()">sign_up</button>
        <router-link to="/">Go back</router-link>
    </div>
    <p>{{ sign_up_info }}</p>
</template>