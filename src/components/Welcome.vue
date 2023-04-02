<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";

const username = ref("");
const password = ref("");
const verify_result = ref("");
const router = useRouter();

async function sign_in() {
  let verify_res = false;
  if (username.value != '' && password.value != '') {
    verify_res = await invoke("sign_in", { username: username.value, password: password.value });
  } else {
    verify_result.value = "Please enter your username or password. "
  }
  if (verify_res) {
    verify_result.value = "verification passed.";
    router.push({ path: `/user/${username.value}` });
  } else {
    verify_result.value = "Please check your username or password.";
  }
}

async function sign_up() {
  router.push({ path: `/sign_up/${username.value}` })
}


</script>

<template>
  <h1>Welcome to Our DDL!</h1>
  <div class="card">
    <input id="username" v-model="username" placeholder="Enter your username..." />
    <input id="password" v-model="password" placeholder="Enter your passwold..." @keyup.enter="sign_in()" />
  </div>
  <div>
    <button id="sign_in" type="button" @click="sign_in()">sign_in</button>
    <button id="sign_up" type="button" @click="sign_up()">sign_up</button>
  </div>

  <p>{{ verify_result }}</p>
  <p></p>
</template>
