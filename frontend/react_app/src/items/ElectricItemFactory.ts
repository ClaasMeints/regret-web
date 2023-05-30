import { capacitor, inductor, resistor } from "../assets/svgs";

enum ElectricItemType {
    RESISTOR = "resistor",
    CAPACITOR = "capacitor",
    INDUCTOR = "inductor",
    VOLTAGE_SOURCE = "voltage_source",
}

// name generation ---------------------------------------------------
const symbols: Record<ElectricItemType, string> = {
    [ElectricItemType.RESISTOR]: "R",
    [ElectricItemType.CAPACITOR]: "C",
    [ElectricItemType.INDUCTOR]: "L",
    [ElectricItemType.VOLTAGE_SOURCE]: "U",
};
var nameCounter: Record<ElectricItemType, number> = {
    [ElectricItemType.RESISTOR]: 0,
    [ElectricItemType.CAPACITOR]: 0,
    [ElectricItemType.INDUCTOR]: 0,
    [ElectricItemType.VOLTAGE_SOURCE]: 0,
};
function getItemName(type: ElectricItemType): string {
    if (!nameCounter[type]) {
        nameCounter[type] = 0;
    }
    nameCounter[type]++;
    return `${symbols[type]}${nameCounter[type]}`;
}
// -------------------------------------------------------------------

export class  ElectricItem {
    private __id: number;
    private __name: string;
    private __svg: string;
    constructor(_id: number, _name: string, _svg: string) {
        this.__id = _id;
        this.__name = _name;
        this.__svg = _svg;
    }

    public get id(): number {
        return this.__id;
    }

    public get name(): string {
        return this.__name;
    }

    public get svg(): string {
        return this.__svg;
    }
}

// unique id
var id = 0;

export function createItem(type: string): ElectricItem | undefined {
    const _type = type as ElectricItemType ?? ElectricItemType.RESISTOR;

    switch (_type) {
        case ElectricItemType.RESISTOR:
            return new ElectricItem(id++, getItemName(_type), resistor);
        case ElectricItemType.CAPACITOR:
            return new ElectricItem(id++, getItemName(_type), capacitor);
        case ElectricItemType.INDUCTOR:
            return new ElectricItem(id++, getItemName(_type), inductor);
        case ElectricItemType.VOLTAGE_SOURCE:
            return new ElectricItem(id++, getItemName(_type), "");
        default:
            
    }
}