<template>
    <EnProcess v-if="locale === 'en'" />
    <ZhProcess v-else />
</template>

<script setup lang="ts">
    const { locale } = useI18n();
</script>
