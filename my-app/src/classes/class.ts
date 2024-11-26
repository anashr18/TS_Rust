
class Cookie {
    private _color: string
    constructor(color: string) {
        this._color = color
    }
    get color(): string {
        return this._color
    }
    set color(newColor: string) {
        this._color = newColor
    }
}

const c1 = new Cookie("blue")
const c2 = new Cookie("black")
console.log(c1.color)
console.log(c2.color)
c2.color = "white"
console.log(c2.color)