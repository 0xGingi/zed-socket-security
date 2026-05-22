/**
 * @fileoverview Centralized path resolution for zed-socket-security.
 *
 * Source of truth for every build/test/runtime path. Per the fleet
 * `1 path, 1 reference` rule — every other module imports from here
 * instead of constructing paths inline.
 *
 * The shipped Zed package is assembled by `zed-package` under
 * `build/zed`, while Rust writes its Wasm component under `target`.
 * This module remains the source of truth for TypeScript-side fleet
 * checks that need repo paths.
 */

import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

// Package root: scripts/../
export const PACKAGE_ROOT = path.resolve(__dirname, '..')
export const REPO_ROOT = PACKAGE_ROOT

export const BUILD_ROOT = path.join(PACKAGE_ROOT, 'build')
export const ZED_BUILD_ROOT = path.join(BUILD_ROOT, 'zed')
export const ZED_PACKAGE_DIR = path.join(ZED_BUILD_ROOT, 'package')
export const ZED_DIST_DIR = path.join(ZED_BUILD_ROOT, 'dist')
export const TARGET_DIR = path.join(PACKAGE_ROOT, 'target')

// Source roots.
export const SRC_DIR = path.join(PACKAGE_ROOT, 'src')

/**
 * Build paths for a specific (mode, platform-arch) tuple.
 *
 * @param buildMode  'dev' | 'prod' (determines minify, debug toggles)
 * @param platformArch  e.g. 'darwin-arm64', 'linux-x64', 'win32-x64'.
 *                       Use 'any' when the artifact is platform-agnostic.
 *
 * Returns an object whose keys mirror the socket-btm canonical:
 *   buildDir         build/<mode>/<platformArch>
 *   outputFinalDir   build/<mode>/<platformArch>/out/Final
 *   outputFinalFile  the packaged Zed archive path
 */
export function getBuildPaths(
  buildMode: 'dev' | 'prod',
  platformArch: string,
): {
  buildDir: string
  outputFinalDir: string
  outputFinalFile: string
} {
  if (!buildMode) {
    throw new Error('buildMode is required for getBuildPaths()')
  }
  if (!platformArch) {
    throw new Error('platformArch is required for getBuildPaths()')
  }
  const buildDir = path.join(BUILD_ROOT, buildMode, platformArch)
  const outputFinalDir = path.join(buildDir, 'out', 'Final')
  return {
    buildDir,
    outputFinalDir,
    outputFinalFile: path.join(outputFinalDir, 'archive.tar.gz'),
  }
}
