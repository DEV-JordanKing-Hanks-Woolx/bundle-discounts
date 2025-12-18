// @ts-check
import { ProductDiscountSelectionStrategy } from '../generated/api';

/**
  * @typedef {import('../generated/api').Input} Input
  * @typedef {import('../generated/api').CartLinesDiscountsGenerateRunResult} CartLinesDiscountsGenerateRunResult
 */

/**
  * @param {Input} input
  * @returns {CartLinesDiscountsGenerateRunResult}
 */
export function cartLinesDiscountsGenerateRun(input) {
  // Parse configuration from metafield
  const configuration = input?.discount?.metafield?.jsonValue ?? {};

  // Return empty operations if no configuration
  if (!configuration || !configuration.target_variant_ids || !configuration.percentage) {
    return { operations: [] };
  }

  const candidates = [];

  // Return empty operations if no cart or lines
  if (!input.cart.lines) {
    return { operations: [] };
  }

  // Process each cart line
  for (const line of input.cart.lines) {
    // Skip if not a product variant
    if (line.merchandise.__typename !== "ProductVariant") {
      continue;
    }

    // Check if variant is in target list
    if (configuration.target_variant_ids.includes(line.merchandise.id)) {
      const message = line.merchandise.title
        ? `${configuration.percentage}% off ${line.merchandise.title}!`
        : `${configuration.percentage}% off this product!`;

      candidates.push({
        value: {
          percentage: {
            value: configuration.percentage.toFixed(1).toString()
          }
        },
        targets: [
          {
            productVariant: {
              id: line.merchandise.id,
              quantity: line.quantity
            }
          }
        ],
        message
      });
    }
  }

  if (candidates.length === 0) {
    return { operations: [] };
  }

  return {
    operations: [
      {
        productDiscountsAdd: {
          candidates,
          selectionStrategy: ProductDiscountSelectionStrategy.First
        }
      }
    ]
  };
}