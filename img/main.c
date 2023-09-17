#include <stdint.h>
#include <stdio.h>
#define cleanup(x) __attribute__((cleanup(x)))

void fclean(FILE **file) {
  if (file != NULL && (*file != NULL)) {
    fclose(*file);
  }
}

int main() {
  cleanup(fclean) FILE *file = fopen("image.rgb", "r");
  cleanup(fclean) FILE *output = fopen("image.rs", "w");
  uint8_t subpixel;
  uint8_t buffer[230400];
  int size;
  int i = 0;
  fprintf(output, "pub(crate) const IMAGE: [u8; 115200] = [\n");
  while ((size = fread(&subpixel, 1, sizeof(subpixel), file)) > 0) {
    buffer[i] = subpixel;
    i++;
  }
  size = i;
  for (uint8_t *pixel = buffer; pixel < &buffer[size]; pixel += 3) {
    pixel[0] ^= pixel[2];
    pixel[2] ^= pixel[0];
    pixel[0] ^= pixel[2];
  }
  uint16_t buf = (buffer[0] >> 3) | ((buffer[1] >> 2) << 5) | ((buffer[2] >> 3) << 11);
  fprintf(output, "    0x%0x, 0x%0x", ((uint8_t *)&buf)[0], ((uint8_t *)&buf)[1]);
  for (int i = 3; i < size; i += 3) {
    buf = (buffer[i] >> 3) | ((buffer[i + 1] >> 2) << 5) | ((buffer[i + 2] >> 3) << 11);
    fprintf(output, ",\n    0x%0x, 0x%0x", ((uint8_t *)&buf)[0], ((uint8_t *)&buf)[1]);
  }
  fprintf(output, "\n];\n");
  return 0;
}
