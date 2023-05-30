import { NodeModel } from "@projectstorm/react-diagrams";

type Item = {
    name: string,
    svg: string,
}

export class SvgNodeModel extends NodeModel {
    private _item: Item;
    constructor(options: {
        item: Item
    }) {
        super({
            type: "electricItem",
        });
        this._item = options.item;
    }

    public get item(): Item {
        return this._item;
    }
}