import update from 'immutability-helper'
import { useCallback, useState } from 'react'
import { useDrop } from 'react-dnd'
import { Node } from './Node.js'
import { ItemTypes } from './ItemTypes.js'
const styles = {
  width: 300,
  height: 300,
  border: '1px solid black',
  position: 'relative',
}
export const Container = ({ hideSourceOnDrag }) => {
  const [Nodes, setNodes] = useState({
    a: { top: 20, left: 80, title: 'Drag me around' },
    b: { top: 180, left: 20, title: 'Drag me too' },
  })
  const moveNode = useCallback(
    (id, left, top) => {
      setNodes(
        update(Nodes, {
          [id]: {
            $merge: { left, top },
          },
        }),
      )
    },
    [Nodes, setNodes],
  )
  const [, drop] = useDrop(
    () => ({
      accept: ItemTypes.NODE,
      drop(item, monitor) {
        const delta = monitor.getDifferenceFromInitialOffset()
        const left = Math.round(item.left + delta.x)
        const top = Math.round(item.top + delta.y)
        moveNode(item.id, left, top)
        return undefined
      },
    }),
    [moveNode],
  )
  return (
    <div ref={drop} style={styles}>
      {Object.keys(Nodes).map((key) => {
        const { left, top, title } = Nodes[key]
        return (
          <Node
            key={key}
            id={key}
            left={left}
            top={top}
            hideSourceOnDrag={hideSourceOnDrag}
          >
            {title}
          </Node>
        )
      })}
    </div>
  )
}
