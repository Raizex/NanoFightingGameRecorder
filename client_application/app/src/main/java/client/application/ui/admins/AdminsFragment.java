package client.application.ui.admins;

import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.TextView;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.fragment.app.Fragment;
import androidx.lifecycle.Observer;
import androidx.lifecycle.ViewModelProvider;

import client.application.R;

public class AdminsFragment extends Fragment {

    private AdminsViewModel adminsViewModel;

    public View onCreateView(@NonNull LayoutInflater inflater,
                             ViewGroup container, Bundle savedInstanceState) {
        adminsViewModel =
                new ViewModelProvider(this).get(AdminsViewModel.class);
        View root = inflater.inflate(R.layout.fragment_admins, container, false);
        final TextView textView = root.findViewById(R.id.text_admins);
        adminsViewModel.getText().observe(getViewLifecycleOwner(), new Observer<String>() {
            @Override
            public void onChanged(@Nullable String s) {
                textView.setText(s);
            }
        });
        return root;
    }
}