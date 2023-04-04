<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute()
const name = ref(route.params.name)
const groups = [
    'Math', 'English', 'Chinese',
]

</script>

<template>
    <h1>Hello, {{ name }}</h1>
    <li v-for="group in groups">
        <router-link :to="`/user/${name}/${group}`">{{ group }}</router-link>
    </li>
    <router-link to="/">sign out</router-link>
</template>