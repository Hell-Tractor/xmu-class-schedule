<script>
export default {
    props: {
        icon: String
    },
    computed: {
        svgContent() {
            const svgFile = require(`@/assets/icons/${this.icon}.svg`);
            return svgFile.default;
        }
    }
}
</script>

<template>
    <div v-html="svgContent" v-if="svgContent"></div>
    <p v-else>Icon Not Found</p>
</template>