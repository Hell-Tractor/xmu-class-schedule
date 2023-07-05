<script>
import ImportFromHtmlDialog from './ImportFromHtmlDialog.vue';

export default {
    data() {
        return {
            showImportFromHtmlDialog: false
        }
    },
    props: {
        modelValue: Boolean
    },
    emits: ["update:modelValue", "schedule-imported"],
    computed: {
        value: {
            get() {
                return this.modelValue;
            },
            set(value) {
                this.$emit("update:modelValue", value);
            }
        }
    },
    methods: {
        importFromHtml() {
            this.showImportFromHtmlDialog = true;
            this.value = false;
        },
        importFromWebsite() {
        },
        emitScheduleImported() {
            this.$emit("schedule-imported");
        }
    },
    components: { ImportFromHtmlDialog }
}
</script>

<template>
    <el-dialog v-model="value" title="导入方式" width="30%">
        <el-button @click="importFromHtml">从HTML文件导入</el-button>
        <el-popover placement="bottom" trigger="hover" content="正在开发中...">
            <template #reference>
                <el-button @click="importFromWebsite">从教务网站导入</el-button>
            </template>
        </el-popover>
    </el-dialog>
    <ImportFromHtmlDialog v-model="showImportFromHtmlDialog" @schedule-imported="emitScheduleImported()"/>
</template>

<style scoped>
.el-dialog__body .el-button,
.el-dialog__body .el-upload {
    width: 100%;
    margin: 5px;
}
</style>