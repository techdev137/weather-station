<template>
  <div>
    <ModeButton
      :edit-mode="editMode"
      :value="mode"
      @input="setMode"
    />
    <CardHeader :editMode="editMode">
      <div>
        {{ sensor.label }}
        <span v-if="sensor.settings">- {{ sensor.station.label }}</span>
      </div>
    </CardHeader>
    <TimeButtons
      :edit-mode="editMode"
      :value="timeAgo"
      @input="setTimeAgo"
      :zoomed-in="zoomedIn"
      @reset-zoom="zoomedIn = false"
    />
    <CurrentView v-if="mode === 'current' && measurements.length" :measurements="measurements">
      <template v-slot:value1>{{ currentHumidity }}%</template>
      <template v-slot:value2>{{ averageHumidity }}%</template>
    </CurrentView>
    <CurrentView v-else-if="mode === 'current'">
      <template v-slot:value1>N/A</template>
      <template v-slot:value2>N/A</template>
    </CurrentView>
    <Graph
      v-else
      :name="sensor.label"
      :measurements="measurements"
      :zoomed-in="zoomedIn"
      @zoomed-in="zoomedIn = true"
    />
    <SortButtons
      v-if="sensor.settings"
      :edit-mode="editMode"
      :sensor-id="sensor.id"
    />
    <BookmarkButton
      :edit-mode="editMode"
      :is-dashboard="!!sensor.settings"
      :mode="mode"
      :sensor-id="sensor.id"
      :time-ago="timeAgo"
    />
  </div>
</template>

<script>
import BookmarkButton from '../BookmarkButton'
import CurrentView from '../CurrentView'
import Graph from '../Graph'
import ModeButton from '../ModeButton'
import SortButtons from '../SortButtons'
import TimeButtons from '../TimeButtons'
import CardHeader from '../CardHeader'

export default {
  components: {
    BookmarkButton,
    CurrentView,
    Graph,
    ModeButton,
    SortButtons,
    TimeButtons,
    CardHeader
  },
  props: {
    editMode: {
      required: true,
      type: Boolean
    },
    sensor: {
      required: true,
      type: Object
    }
  },
  data() {
    return {
      // Hydrate from sensor "settings" if this is on the dashboard
      mode: this.sensor.settings?.mode || 'current',
      timeAgo: this.sensor.settings?.timeAgo || 1728e5,
      zoomedIn: false
    }
  },
  computed: {
    averageHumidity() {
      const sum = this.measurements.reduce((acc, el) => acc + Number(el.value), 0)
      return Math.round((sum / this.measurements.length) * 10) / 10
    },
    currentHumidity() {
      if (this.measurements.length) {
        return this.measurements[this.measurements.length - 1].value
      }
      return 0
    },
    measurements() {
      if (this.timeAgo === Infinity) {
        return this.sensor.measurements
      }
      const now = new Date().getTime()
      return this.sensor.measurements
        // Filter down to the last 48 hours
        .filter(m => now - Math.round(new Date(m.created_at).getTime()) <= this.timeAgo)
    }
  },
  methods: {
    setMode(newMode) {
      this.mode = newMode
      // State of graph gets reset with mode changes
      this.zoomedIn = false
      this.$emit('change-mode', this.mode)
    },
    setTimeAgo(timeAgo) {
      this.timeAgo = timeAgo
      this.$emit('change-time-ago', this.timeAgo)
    }
  }
}
</script>
