<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute()
const name = ref(route.params.name)
const group = ref(route.params.group)


</script>

<template>
    <h1>Hello, {{ name }}</h1>
    <h2>This is group {{ group }}</h2>
    <router-link :to="`/user/${name}`">back</router-link>
</template>