<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute()
const path = ref(route.params.pathMatch)

</script>

<template>
    <h1>404 Not Found</h1>
    <div>{{ path }}</div>
    <router-link to="/">Go back</router-link>
</template>