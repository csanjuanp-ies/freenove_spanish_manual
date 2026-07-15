mod lib_cap{
    pub fn number_to_display(number:u8 ,matriz:&mut [[u8;5];5]){
        match number{
            0 =>
                *matriz = [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0]],
            1 => {
                *matriz = [
                    [0, 0, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0]];
            },
            2 => {
                *matriz = [
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 0],
                    [0, 1, 1, 1, 0]];
            },
            _ => {
                *matriz = [
                    [0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0],
                    [1, 1, 1, 1, 1],
                    [0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0]];
            }
        }
    }

    pub fn rotate_column_matrix(matriz:&mut [[u8;5];5]){
        for col in (1..5).rev() {
            matriz[0][col] = matriz[0][col-1];
            matriz[1][col] = matriz[1][col-1];
            matriz[2][col] = matriz[2][col-1];
            matriz[3][col] = matriz[3][col-1];
            matriz[4][col] = matriz[4][col-1];
        }
        {
            matriz[0][0] = 0;
            matriz[1][0] = 0;
            matriz[2][0] = 0;
            matriz[3][0] = 0;
            matriz[4][0] = 0;
        }
    }
}