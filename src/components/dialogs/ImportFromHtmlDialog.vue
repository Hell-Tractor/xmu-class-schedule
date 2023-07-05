<script>
import { Delete } from '@element-plus/icons-vue';
import { UploadFilled, Loading, Check, Close } from '@element-plus/icons-vue';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';

export default {
    data() {
        return {
            files: []
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
        },
        allFileUploaded() {
            for (let file of this.files) {
                if (file.state != "success" && file.state != "error")
                    return false;
            }
            return true;
        },
    },
    methods: {
        async selectFile() {
            this.files = this.files.concat((await open({
                multiple: true,
                filters: [{
                    name: "HTML文件",
                    extensions: ["html"]
                }]
            })).map(file => {
                return { path: file, state: "ready" };
            })).filter((file, index, self) => {
                return self.findIndex(f => f.path == file.path) == index;
            });
        },
        deleteFile(id) {
            this.files.splice(id, 1);
        },
        async execute() {
            if (this.allFileUploaded) {
                this.files = [];
                this.value = false;
                this.$emit("schedule-imported");
                return;
            }
            for (let file of this.files) {
                if (file.state != "ready")
                    continue;
                file.state = "uploading";
                try {
                    await invoke("parse_schedule", { inputFilePath: file.path, name: file.path.split('\\').slice(-1)[0].split('.')[0] });
                    file.state = "success";
                } catch (error) {
                    console.warn(`Failed to parse html ${file.path} to schedule: ${error}`);
                    file.state = "error";
                }
            }
        },
    },
    components: { UploadFilled, Delete, Loading, Check, Close }
}
</script>

<template>
    <el-dialog v-model="value" title="导入课程表" width="50%">
        <el-button @click="selectFile">选择HTML文件</el-button>
        <ul class="filelist">
            <li v-for="(file, index) in files">
                {{ file.path.split('\\').slice(-1)[0] }}
                <el-button v-if="file.state == 'ready'" type="danger" circle size="small" @click="deleteFile(index)"><el-icon><Delete/></el-icon></el-button>
                <el-icon v-else-if="file.state == 'uploading'"><Loading /></el-icon>
                <el-icon v-else-if="file.state == 'success'" color="green"><Check /></el-icon>
                <el-icon v-else color="red"><Close /></el-icon>
            </li>
        </ul>
        <template #footer>
            <el-button type="primary" @click="execute">{{ allFileUploaded ? "确定" : "导入" }}</el-button>
        </template>
    </el-dialog>
</template>

<style scoped>
.filelist {
    list-style: none;
    padding-left: 5px;
}

.filelist > li {
    margin-top: 5px;
}

.filelist .el-button,
.filelist .el-icon {
    float: right;
}

.filelist > .el-icon {
    margin-top: 5px;
}
</style>