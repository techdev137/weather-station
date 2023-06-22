<template>
  <div class="flex-container">
    <v-card
      class="flex-item"
      elevation="0"
      outlined
      rounded
      v-for="sensor in sensors"
      :key="sensor.id">
      <TemperatureCard
        v-if="sensor.type.label === 'temperature'"
        :sensor="sensor"
        class="card"
      />
      <div v-else class="card">
        Invalid sensor type!
      </div>
    </v-card>
  </div>
</template>

<script>
import TemperatureCard from './cards/Temperature'

export default {
  components: {
    TemperatureCard
  },
  props: {
    sensors: {
      required: true,
      type: Array
    }
  },
  methods: {
    changeMode(sensorId, mode) {
      this.$emit('change-mode', { sensorId, mode })
    },
    changeTimeAgo(sensorId, timeAgo) {
      this.$emit('change-time-ago', { sensorId, timeAgo })
    }
  }
}
</script>

<style scoped>
.flex-container {
  display: flex;
  flex-flow: row wrap;
  justify-content: space-around;
}

.flex-item {
  flex-basis: 25%;
  flex-grow: 0.35;
  flex-shrink: 0;
  min-height: 20em;
  width: 25%;
}

.flex-item:first-child {
  margin-top: 2px;
}

@media screen and (max-width: 1800px) {
  .flex-item {
    flex-basis: 33.33%;
    width: 33.33%;
  }
}

@media screen and (max-width: 860px) {
  .flex-item {
    flex-basis: 50%;
    width: 50%;
  }
}
@media screen and (max-width: 640px) {
  .flex-item {
    flex-basis: 100%;
    min-height: 120px;
    width: 100%;
  }
}

/deep/ .bookmark-button {
  bottom: 0;
  display: none;
  padding-bottom: 4px;
  padding-right: 4px;
  position: absolute;
  right: 0;
  z-index: 1;
}

.card {
  height: 100%;
}

/deep/ .card:hover .bookmark-button {
  display: block;
}

@media screen and (max-width: 640px) {
  .card {
  }
}
</style>