import { SvgNodeModel } from "./Model";
import { DiagramEngine } from "@projectstorm/react-diagrams";

export interface SvgWidgetProps {
    node: SvgNodeModel;
    engine: DiagramEngine;
    size: number;
}

export function SvgNodeWidget(props: SvgWidgetProps) {
    return (
        <div
            className={'diamond-node'}
            style={{
                position: 'relative',
                width: props.size,
                height: props.size
            }}>
            <div
                style={{
                    position: 'absolute',
                    top: props.size * 3 / 4,
                    left: 0,
                    width: props.size,
                    textAlign: 'center',
                    fontSize: 20,
                    fontWeight: 'bold'
                }}>
                {props.node.item.name}
            </div>
            <svg
                width={props.size}
                height={props.size}
                dangerouslySetInnerHTML={{
                    __html: props.node.item.svg
                }} />
        </div>

    );
};