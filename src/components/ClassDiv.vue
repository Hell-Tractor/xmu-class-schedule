<script>
export default {
    data() {
        return {
        }
    },
    props: {
        classData: Object,
        color: {
            type: String,
            default: "#000"
        }
    },
}
</script>

<template>
    <div class="class" v-if="classData != null" :style="{ backgroundColor: color }">
        <h1>{{ classData.name }}</h1>
        <div>{{ classData.location }}</div>
        <div>{{ classData.teacher }}</div>
    </div>
</template>

<style scoped>
.class {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100%;
    width: 100%;
    border-radius: 10px;
    padding: 0px 5px 5px 5px;
}
.class > h1 {
    color: #fff;
    text-align: center;
    font-size: 1em;
}
.class > div {
    color: #fff;
}
</style>