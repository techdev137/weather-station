<template>
  <v-btn
    @click="switchMode()"
    class="v-btn-toggle mode-button"
    :class="{ 'edit-mode': editMode }"
    elevation="0"
    outlined
    x-small
  >
    <v-icon x-small>{{ icon(nextItem) }}</v-icon>
  </v-btn>
</template>

<script>
export default {
  props: {
    editMode: {
      required: true,
      type: Boolean
    },
    modes: {
      default: () => (['current', 'chart']),
      type: Array
    },
    value: {
      required: true,
      type: String
    }
  },
  computed: {
    nextItem() {
      const index = this.modes.findIndex(v => v === this.value)
      return this.modes[index + 1] || this.modes[0]
    }
  },
  methods: {
    icon(mode) {
      const map = {
        chart: 'mdi-chart-timeline-variant',
        current: 'mdi-counter',
        'percentage-chart': 'mdi-battery-60'
      }
      return map[mode]
    },
    switchMode() {
      this.$emit('input', this.nextItem)
    }
  }
}
</script>

<style scoped>
.mode-button {
  display: none;
  margin-top: 2px;
  position: absolute;
  left: 2px;
  z-index: 1;
}

.mode-button.edit-mode {
  display: block;
}

.mode-button.theme--light {
border-color: var(--v-secondary-lighten5);
}

.mode-button.theme--dark {
border-color: var(--v-secondary-base);
}

@media screen and not (max-width: 1000px) {
  .mode-button {
    display: block;
  }
}
</style>
