package grid

import kotlin.math.pow
import kotlin.math.sqrt

data class Position3D(
  val x: Int,
  val y: Int,
  val z: Int,
) {
  fun distanceSquared(other: Position3D): Double =
    (x - other.x).toDouble().pow(2) +
      (y - other.y).toDouble().pow(2) +
      (z - other.z).toDouble().pow(2)

  @Suppress("UNUSED")
  fun distanceTo(other: Position3D): Double =
    sqrt(
      this.distanceSquared(other).toDouble(),
    )
}
