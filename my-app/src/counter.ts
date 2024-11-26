export function setupCounter(element: HTMLButtonElement) {
  let counter = 0
  const setCounter = (count: number) => {
    counter = count
    element.innerHTML = `count is ${counter}`
  }
  element.addEventListener('click', () => setCounter(counter + 1))
  setCounter(0)
}

// class Cookie {
//   private _color: string
//   constructor(color: string) {
//       this._color = color
//   }
//   get color(): string {
//       console.log('in getter')
//       return this._color
//   }
//   set color(newcolor: string) {
//       console.log('in setter')
//       this._color = newcolor
//   }
// }

// const ck1 = new Cookie('blue')
// const ck2 = new Cookie('red')

// ck1.color = 'white'
// console.log(ck1.color)
// console.log(ck2.color)