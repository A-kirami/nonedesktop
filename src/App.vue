<script setup lang="ts">
import { locale as dayjsLocale } from 'dayjs'

import 'dayjs/locale/zh-cn'

dayjsLocale('zh-cn')

onBeforeMount(async () => {
  await logger.attachConsole()
})
</script>

<template>
  <RouterView />
</template>
