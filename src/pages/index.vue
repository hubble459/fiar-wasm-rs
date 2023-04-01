<template>
    <h1>100 fps (raytracing on)</h1>
    <div
        :key="frame.toString()"
        class="grid grid-cols-7 align-middle place-items-center gap-2"
    >
        <button
            v-for="(cell, index) of cells"
            :key="index"
            class="w-20 h-20 bg-gray-600 rounded"
            :class="{
                '!bg-blue-500': cell === Cell.PlayerOne,
                '!bg-red-500': cell === Cell.PlayerTwo,
            }"
            @click="move(index, Cell.PlayerOne)"
            @contextmenu.prevent.stop="move(index, Cell.PlayerTwo)"
        >
            {{ cell === Cell.Empty ? 'X' : cell === Cell.PlayerOne ? '1' : '2' }} ({{ index }})
        </button>
    </div>
</template>

<script setup lang="ts">
    import init, { FourInARow, Cell } from 'rust';

    const cells = ref(new Uint8Array());
    let fiar!: FourInARow;
    const frame = ref(0);

    init().then(({ memory }) => {
        fiar = FourInARow.new();
        const cellsPtr = fiar.cells();
        cells.value = new Uint8Array(memory.buffer, cellsPtr, fiar.width * fiar.height);

        console.log(cells.value);

        for (let index = 0; index < 7; index += 1) {
            const a = fiar.get_index(index);
            console.log(a);
        }

        // for (let index = 0; index < cells.value.length; index++) {
        // }
    });

    function move(index: number, player: Cell) {
        console.log({ index, frame: frame.value });
        console.log(index % 7);

        fiar.set_index(index, player);
        frame.value += 1;

        console.log(fiar.is_game_over());
    }

    const { t } = useI18n();
    ElMessage.success({ message: 'welcome', duration: 1000 });
    ElNotification({
        title: 'Issue',
        message: 'If you encounter problems in using the template, please raise them in the issue',
        duration: 0,
    });
    const featureList = [
        {
            name: 'Vite3',
            href: 'https://github.com/vitejs/vite',
        },
        {
            name: 'Vue3',
            href: 'https://github.com/vuejs/core',
        },
        {
            name: 'TypeScript',
            href: 'https://github.com/microsoft/TypeScript',
        },
        {
            name: 'Element Plus',
            href: 'https://element-plus.gitee.io/zh-CN/',
        },
        {
            name: 'Vue Router 4',
            href: 'https://router.vuejs.org/zh/',
        },
        {
            name: 'Pinia',
            href: 'https://pinia.vuejs.org/',
        },
        {
            name: 'icones',
            href: 'https://icones.netlify.app/',
        },
        {
            name: 'Windi CSS',
            href: 'https://github.com/windicss/windicss',
        },
        {
            name: 'Axios',
            href: 'https://axios-http.com/',
        },
        {
            name: 'I18n',
            href: 'https://github.com/intlify/vite-plugin-vue-i18n',
        },
        {
            name: 'Prettier',
            href: 'https://github.com/prettier/prettier',
        },
        {
            name: 'ESLint',
            href: 'https://github.com/eslint/eslint',
        },
        {
            name: 'Airbnb Style',
            href: 'https://github.com/airbnb/javascript',
        },
        {
            name: 'Husky',
            href: 'https://github.com/typicode/husky',
        },
        {
            name: 'VueUse',
            href: 'https://github.com/vueuse/vueuse',
        },
        {
            name: 'Markdown',
            href: 'https://github.com/antfu/vite-plugin-md',
        },
        {
            name: 'NProgress',
            href: 'https://github.com/rstacruz/nprogress',
            isEnd: true,
        },
    ];
</script>

<style lang="scss" scoped>
a {
  @apply text-sky-400 hover:(text-sky-600) transition-all ease-out duration-100;
}
</style>
