<script>
import Schedule from "./components/Schedule.vue";
import { Menu, ArrowLeft, ArrowRight } from '@element-plus/icons-vue'

export default {
  data() {
    return {
      weekId: 1,
      today: {},
      startOfSemester: new Date(2023, 1, 13),
      showMenu: false
    }
  },
  components: {
    Schedule,
    Menu,
    ArrowLeft,
    ArrowRight
  },
  methods: {
    getToday() {
      var today = new Date();
      var weeks = Math.floor((today - this.startOfSemester) / (7 * 24 * 60 * 60 * 1000)) + 1;
      var day = today.getDay();
      return { week: weeks, day: day };
    }
  },
  mounted() {
    this.today = this.getToday();
    this.weekId = this.today.week;
  }
}
</script>

<template>
  <el-row>
    <el-col :span="3" class="title-bar">
      <el-button type="default" class="title-bar no-border" @click="showMenu = !showMenu"><el-icon><Menu /></el-icon></el-button>
    </el-col>
    <el-col :span="17" class="title-bar">
      <div class="title-text">
        第{{ weekId }}周
      </div>
    </el-col>
    <el-col :span="4" class="title-bar flex-align-right">
      <el-button type="default" class="title-bar no-border" @click="weekId--;" v-if="weekId > 1"><el-icon><ArrowLeft /></el-icon></el-button>
      <el-button type="default" class="title-bar no-border" @click="weekId++;"><el-icon><ArrowRight /></el-icon></el-button>
    </el-col>
  </el-row>
  <el-row>
    <el-col :span="24">
      <Schedule v-if="!showMenu" :weekId="weekId" scheduleName="2022-2023 夏季学期" :showWeekends="false" :today="today"/>
    </el-col>
  </el-row>
  <!-- <ClassBg :classData="123"></ClassBg> -->
</template>

<style scoped>
.title-bar {
  min-height: 40px;
  max-height: 40px;
  background-color: white;
}

.title-text {
  font-size: 1.5em;
  padding-top: 10px;
}

.no-border {
  border: none;
  box-shadow: none;
}

.flex-align-right {
  display: flex;
  justify-content: flex-end;
}
</style>