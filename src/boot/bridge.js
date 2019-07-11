import { uid } from 'quasar'
import EventEmitter from 'events'

class Bridge extends EventEmitter {
  constructor () {
    super()
    this.promises = new Map()
  }
  receive (data) {
    switch (data.subtype) {
      case 'PROMISE_RENDER':
        if (data.id && this.promises.has(data.id)) {
          if (data.status === 'RESOLVE') {
            this.promises.get(data.id)[0](data.message)
          } else {
            this.promises.get(data.id)[1](data.message)
          }
          this.promises.delete(data.id)
        }
        break
      case 'PROMISE_MAIN':
        break
      case 'TRANSMIT':
        this.emit('message', data.message)
        break
    }
  }
  send (data) {
    window.external.invoke(JSON.stringify({ cmd: 'message', data }))
  }
  sendPromise (data) {
    return new Promise((resolve, reject) => {
      const id = uid()
      this.promises.set(id, [resolve, reject])
      window.external.invoke(JSON.stringify({ cmd: 'messagePromise', id, data }))
    })
  }
}

export default ({ Vue }) => {
  Vue.prototype.$bridge = window.bridge = new Bridge()
}
