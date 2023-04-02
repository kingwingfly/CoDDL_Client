<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute()
const name = ref(route.params.name)

</script>

<template>
    <h1>Hello, {{ name }}</h1>
    <router-link to="/">sign out</router-link>
</template>