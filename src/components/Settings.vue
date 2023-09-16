<script>
import { invoke } from '@tauri-apps/api';

export default {
    data() {
        return {
            startOfSemester: new Date(2023, 2, 13),
            defaultScheduleName: ""
        }
    },
    props: {
        scheduleList: Object,
        showSettings: Boolean
    },
    emits: [ "update:showSettings", "update" ],
    computed: {
        showSettings: {
            get() {
                return this.showSettings;
            },
            set(value) {
                this.$emit("update:showSettings", value);
            }
        }
    },
    methods: {
        async save_settings() {
            var settings = {
                startOfSemester: this.startOfSemester,
                defaultScheduleName: this.defaultScheduleName
            };
            this.$emit("update", settings);
            await invoke("set_settings", { settings: settings });
        },
        async load_settings(emitEvent = false) {
            var settings = await invoke("get_or_default_settings");
            this.startOfSemester = settings["startOfSemester"];
            this.defaultScheduleName = settings["defaultScheduleName"];
            if (emitEvent)
                this.$emit("update:settings", settings);
        }
    },
    mounted() {
        this.load_settings(true);
    }
}
</script>

<template>
    <el-dialog v-model="showSettings" title="设置" width="50%" @close="save_settings" @open="load_settings">
        <el-row>
            <el-col :span="8">
                <div class="setting-text">
                    学期开始时间
                </div>
            </el-col>
            <el-col :span="16">
                <el-date-picker v-model="startOfSemester" type="date" placeholder="请选择日期" />
            </el-col>
        </el-row>
        <el-row>
            <el-col :span="8">
                <div class="setting-text">
                    默认课程表
                </div>
            </el-col>
            <el-col :span="16">
                <el-select v-model="defaultScheduleName" placeholder="请选择">
                    <el-option v-for="s in scheduleList" :key="s" :label="s" :value="s" />
                </el-select>
            </el-col>
        </el-row>
    </el-dialog>
</template>

<style scoped>
.setting-text {
    padding-top: 5px;
    font-size: 16px;
}
</style>