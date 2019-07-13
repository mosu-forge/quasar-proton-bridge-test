<template>
  <q-page class="flex flex-center">
    <img alt="Quasar logo" src="~assets/quasar-logo-full.svg">
  </q-page>
</template>

<style>
</style>

<script>
export default {
  name: 'PageIndex',
  mounted () {
    this.$bridge.on('message', message => {
      console.log('message received', JSON.stringify(message))
    })

    setTimeout(() => {
      this.$bridge.send('ping')

      this.$bridge.sendPromise('foobar').then(message => {
        console.log('sendPromise was resolved', message)
      }).catch(error => {
        console.error('sendPromise was rejected', error)
      })

      this.$bridge.sendPromise('rejectme').then(message => {
        console.log('sendPromise was resolved', message)
      }).catch(error => {
        console.error('sendPromise was rejected', error)
      })
    }, 1000)
  }
}
</script>
