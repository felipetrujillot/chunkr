import subprocess
import os
import tempfile
import base64
from pathlib import Path
from typing import Dict

from utils import needs_conversion

def convert_to_img(file: Path, density: int) -> Dict[int, str]:
    temp_dir = tempfile.mkdtemp()
    result = {}
    try:
        if needs_conversion(file):
            subprocess.run(['libreoffice', '--headless', '--convert-to', 'pdf', '--outdir', temp_dir, str(file)],
                           check=True, capture_output=True, text=True)
            pdf_file = next(Path(temp_dir).glob('*.pdf'))
        else:
            pdf_file = file

        output_pattern = os.path.join(temp_dir, 'output-%d.png')
        subprocess.run(['magick', str(pdf_file), '-density', str(density), output_pattern],
                       check=True, capture_output=True, text=True)
        
        for img_file in sorted(os.listdir(temp_dir)):
            if img_file.startswith('output-') and img_file.endswith('.png'):
                page_num = int(img_file.split('-')[1].split('.')[0])
                with open(os.path.join(temp_dir, img_file), 'rb') as img:
                    img_base64 = base64.b64encode(img.read()).decode('utf-8')
                    result[page_num] = img_base64
        
        return result
    except subprocess.CalledProcessError as e:
        raise RuntimeError(f"Failed to convert image: {e.stderr}")
    finally:
        import shutil
        shutil.rmtree(temp_dir)