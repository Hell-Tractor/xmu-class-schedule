<script>
import { invoke } from '@tauri-apps/api/tauri';
import ClassDiv from './ClassDiv.vue';

export default {
    data() {
        return {
            month: 3,
            weekdays: ["周一", "周二", "周三", "周四", "周五"],
            weekends: ["周六", "周日"],
            classes: [],
            classColors: {},
            colorId: 0,
            isLoading: false,
        };
    },
    props: {
        showWeekends: {
            type: Boolean,
            default: false
        },
        weekId: Number,
        scheduleName: String,
        colorSet: {
            type: Array,
            default: () => ["#f56c6c", "#e6a23c", "#409eff", "#67c23a", "#909399"]
        },
        today: Object
    },
    computed: {
        days() {
            return this.weekdays.concat(this.showWeekends ? this.weekends : []);
        },
    },
    watch: {
        weekId() {
            this.getClasses();
        },
        scheduleName() {
            this.getClasses();
        }
    },
    methods: {
        async getClasses() {
            if (this.scheduleName == "无课程表")
                return;
            this.isLoading = true;
            try {
                const classes = await invoke("get_schedules", { name: this.scheduleName, week: this.weekId });
                this.classes = classes;
            } catch (error) {
                console.error(error);
            } finally {
                this.isLoading = false;
            }
        },
        spanMethod({row, column, rowIndex, columnIndex}) {
            var span = this.classes[rowIndex][columnIndex];
            if (span == undefined)
                return [1, 1];
            return [span["span"], 1];
        },
        getClassColor(name) {
            // if (name == undefined)
            //     return "#000";
            if (this.classColors[name] == undefined) {
                this.classColors[name] = this.colorSet[this.colorId];
                this.colorId = (this.colorId + 1) % this.colorSet.length;
            }
            return this.classColors[name];
        },
        shouldHighLight(dayIndex) {
            if (this.weekId == this.today.week && dayIndex == this.today.day)
                return "highlight";
            return "";
        }
    },
    mounted() {
        this.getClasses();
    },
    components: { ClassDiv }
}
</script>

<template>
    <el-table v-loading="isLoading" :data="classes" border :span-method="spanMethod" style="width: 100%">
        <el-table-column :label="month + '月'" width="65" align="center">
            <template #default="scope">
                {{ scope.row.time[0] }} <br> {{ scope.row.time[1] }}
            </template>
        </el-table-column>
        <el-table-column v-for="(day, index) in days" align="center">
            <template #header>
                <div :class="shouldHighLight(index + 1)">{{ day }}</div>
            </template>
            <template #default="scope" class="classgrid">
                <ClassDiv :classData="scope.row[index + 1]" :color="getClassColor(scope.row[index + 1]?.name)"></ClassDiv>
            </template>
        </el-table-column>
    </el-table>
</template>

<style scoped>
.highlight {
    background-color: #e9e7e7;
    border-radius: 10px;
}
</style>